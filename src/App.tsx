import VideoFeed from "./components/VideoFeed";
import CanvasDisplay from "./components/CanvasDisplay";
import { useWebcam } from "./hooks/useWebcam";

const App = () => {
    const width = 320;
    const height = 240;
    const videoStream = useWebcam({width, height});

    return (
        <div>
            <h1>Face Detection App</h1>
            <VideoFeed stream={videoStream} />
            <CanvasDisplay videoStream={videoStream} width={width} height={height} />
        </div>
    );
};

export default App;
