type Props = {
  fps: number
}

const FpsDisplay = ({ fps }: Props) => {
    return <div id="fps-display">FPS: {fps}</div>;
};

export default FpsDisplay;
