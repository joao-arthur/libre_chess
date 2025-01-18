import type {  ReactElement } from "react";
import {  useRef } from "react";
import { useWindowDimension } from "../hooks/useWindowDimension";

export default function Main(): ReactElement {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const dimension = useWindowDimension();

    return (
        <main className="w-screen h-screen flex">
            <canvas
                onClick={() => {}}
                className="m-auto"
                width={dimension}
                height={dimension}
                style={{ width: dimension, height: dimension }}
                ref={canvasRef}
            />
        </main>
    );
}
