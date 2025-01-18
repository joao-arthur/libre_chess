import type {  ReactElement } from "react";
import {  useEffect, useRef } from "react";
import { useWindowDimension } from "../hooks/useWindowDimension";
import initWASM, { engineInit } from "libre_chess_engine";

export default function Main(): ReactElement {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const dimension = useWindowDimension();

    useEffect(() => {
        setTimeout(() => {
            initWASM().then(() => {
                if (!canvasRef.current) {
                    return;
                }
                const context = canvasRef.current.getContext("2d");
                if (!context) {
                    return;
                }
                engineInit(context);
            });
        }, 3000);
    })

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
