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
    const [blockSize, setBlockSize] = useState(10);

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

        const drawImage = async () => {
            if (!ctx) return;

            ctx.drawImage(video, 0, 0, width, height);
            const rgba = ctx.getImageData(0, 0, width, height).data;

            const detectedData = detect_bounding_box(new Uint8Array(rgba), width, height, blockSize);
            detectedData.forEach((info: BboxInfo) => {
                const top = info.x();
                const left = info.y();

                info.mosaic.forEach((row, j) => {
                    row.cols().forEach((rgb, i) => {
                        const x = top + i * blockSize;
                        const y = left + j * blockSize;
                        ctx.fillStyle = `rgb(${rgb.r}, ${rgb.g}, ${rgb.b})`;
                        ctx.fillRect(x, y, blockSize, blockSize);
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
    }, [videoStream, blockSize, width, height]);

    const handleSliderChange: ChangeEventHandler<HTMLInputElement> = (e) => {
        setBlockSize(Number(e.target.value));
    };

    return (
        <div>
            <canvas ref={canvasRef} width={width} height={height} />
            <input
                type="range"
                min="5"
                max="50"
                value={blockSize}
                ref={sliderRef}
                onChange={handleSliderChange}
            />
            <div>Block size: {blockSize}</div>
            <div>FPS: {fps}</div>
        </div>
    );
};

export default CanvasDisplay;
