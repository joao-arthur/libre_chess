import type { ReactElement } from "react";
import { useEffect, useRef } from "react";
import { useWindowDimension } from "../hooks/useWindowDimension";
import initWASM from "libre_chess_wasm";
import { useChess } from "../hooks/useChess";

export default function Main(): ReactElement {
    const {
        init,
        //model
    } = useChess();
    const initiated = useRef(false);
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const dimension = useWindowDimension();

    useEffect(() => {
        if (!initiated.current) {
            initiated.current = true;
            initWASM().then(() => {
                if (!canvasRef.current) {
                    return;
                }
                init(canvasRef.current);
            });
        }
    }, []);

    return (
        <main className="w-screen h-screen flex">
            <canvas
                className="m-auto"
                width={dimension}
                height={dimension}
                style={{ width: dimension, height: dimension }}
                ref={canvasRef}
            />
        </main>
    );
}
