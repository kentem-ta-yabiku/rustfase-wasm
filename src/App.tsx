import VideoFeed from "./components/VideoFeed";
import CanvasDisplay from "./components/CanvasDisplay";
import { useWebcam } from "./hooks/useWebcam";
import styles from "./App.module.css";

const App = () => {
    const width = 320;
    const height = 240;
    const videoStream = useWebcam({width, height});

    return (
        <div className={styles.appContainer}>
            <h1 className={styles.title}>映る者、皆モザイク</h1>
            <VideoFeed stream={videoStream} />
            <CanvasDisplay videoStream={videoStream} width={width} height={height} />
        </div>
    );
};

export default App;

