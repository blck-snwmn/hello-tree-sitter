interface Props {
    name: string;
}

const Component: React.FC<Props> = ({ name }) => {
    return <div>Hello {name}</div>;
};

function AnotherComponent(props: Props) {
    return <span>{props.name}</span>;
}

class ClassComponent {
    render() {
        return <p>TSX</p>;
    }
}

export default Component;
