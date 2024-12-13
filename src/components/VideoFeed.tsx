import { useRef, useEffect } from "react";

type Props = {
  stream?: MediaStream
}

const VideoFeed = ({ stream }: Props) => {
    const videoRef = useRef<HTMLVideoElement>(null);

    useEffect(() => {
        if (videoRef.current && stream) {
            videoRef.current.srcObject = stream;
        }
    }, [stream]);

    return <video ref={videoRef} autoPlay muted id="video-feed" />;
};

export default VideoFeed;
