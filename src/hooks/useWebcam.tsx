import { useState, useEffect } from "react";

type Props = {
    width: number
    height: number
}

export const useWebcam = ({width, height}: Props) => {
    const [videoStream, setVideoStream] = useState<MediaStream>();

    useEffect(() => {
        const getStream = async () => {
            const constraints = {
                video: { width, height, facingMode: { exact: "user" } },
            };
            try {
                const stream = await navigator.mediaDevices.getUserMedia(constraints);
                setVideoStream(stream);
            } catch (err) {
                console.error("Error accessing webcam:", err);
            }
        };
        getStream();
    }, [width, height]);

    return videoStream;
};
