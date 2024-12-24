import { useRef, useEffect, useState, ChangeEventHandler } from "react";
import init, { setup_detector, detect_bounding_box } from "./../../wasm/pkg/rustfase_detection";

type Props = {
  videoStream?: MediaStream
  width: number
  height: number
}

type BboxInfo = {
  x: () => number
  y: () => number
  mosaic: Row[]
}

type Row = {
  cols: () => Rgb[]
}

type Rgb = {
  r: number
  g: number
  b: number
}

const CanvasDisplay = ({ videoStream, width, height }: Props) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const sliderRef = useRef(null);
    const [fps, setFps] = useState<number>();
    const blockSize = useRef(10);
    const isMosaic = useRef(true);
    const [imageData, setImageData] = useState<ImageData>()

    const createRedImage = () => {
      const size = 500 * 500 * 4;
  
      // 赤色のUint8Arrayを直接作成（RGBA = 255, 0, 0, 255）
      const imageData = new Uint8Array(size).fill(0);  // 全て0で初期化
      for (let i = 0; i < size; i += 4) {
        imageData[i] = 255;  // R (赤)
        imageData[i + 3] = 255;  // A (アルファ)
      }

      return imageData;
    }

    const convertImageData = (file: File) => {
      const reader = new FileReader();
      reader.onload = (event) => {
        const img = new Image();
        img.onload = () => {
          // Canvasに画像を描画
          const canvas = document.createElement('canvas');
          const ctx = canvas.getContext('2d');
          if (ctx) {
            canvas.width = img.width;
            canvas.height = img.height;
            ctx.drawImage(img, 0, 0);
  
            // ImageDataを取得
            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            setImageData(imageData);  // stateに保存
          }
        };
        img.src = event.target?.result as string;
      };
      reader.readAsDataURL(file);
    }

    useEffect(() => {
        if (!videoStream) return;
        const canvas = canvasRef.current;
        if (!canvas) return;

        const ctx = canvas.getContext("2d");
        const video = document.getElementById("video-feed") as CanvasImageSource;
        let lastTime: number;

        const showFps = () => {
            if (!lastTime) {
                lastTime = performance.now();
                return;
            }
            const delta = (performance.now() - lastTime) / 1000;
            lastTime = performance.now();
            setFps(Math.floor(1 / delta));
        };

        console.log(imageData?.data);
        const drawImage = async () => {
            if (!ctx) return;

            ctx.drawImage(video, 0, 0, width, height);
            const rgba = ctx.getImageData(0, 0, width, height).data;
            const overlayImage = imageData?.data ?? createRedImage();
            const bs = isMosaic ? blockSize.current : 1;
            let detectedData: BboxInfo[] = detect_bounding_box(new Uint8Array(rgba), width, height, bs, isMosaic.current, new Uint8Array(overlayImage));

            detectedData.forEach((info: BboxInfo) => {
                const top = info.x();
                const left = info.y();

                info.mosaic.forEach((row, j) => {
                    row.cols().forEach((rgb, i) => {
                        const x = top + i * bs;
                        const y = left + j * bs;
                        ctx.fillStyle = `rgb(${rgb.r}, ${rgb.g}, ${rgb.b})`;
                        ctx.fillRect(x, y, bs, bs);
                    });
                });
            });

            showFps();
            requestAnimationFrame(drawImage);
        };

        const initialize = async () => {
            await init();
            setup_detector(20, 2.8, 0.5, 4);
            drawImage();
        };

        initialize();

    }, [videoStream, blockSize, width, height, imageData]);

    const handleSliderChange: ChangeEventHandler<HTMLInputElement> = (e) => {
        blockSize.current = Number(e.target.value);
    };

    const handleCheck: ChangeEventHandler<HTMLInputElement> = (e)=> {
      isMosaic.current = e.target.checked;
    }

    const handleUploadChange: ChangeEventHandler<HTMLInputElement> = (e) => {
      const file = e.target.files?.[0];
      if (!file) return;
      convertImageData(file)
    }

    return (
        <div>
            <canvas ref={canvasRef} width={width} height={height} />
            <input type="checkbox" checked={isMosaic.current} onChange={handleCheck}/>
            <input type="file" onChange={handleUploadChange} />
            <input
                type="range"
                min="5"
                max="50"
                value={blockSize.current}
                ref={sliderRef}
                onChange={handleSliderChange}
            />
            <div>Block size: {blockSize.current}</div>
            <div>FPS: {fps}</div>
        </div>
    );
};

export default CanvasDisplay;
