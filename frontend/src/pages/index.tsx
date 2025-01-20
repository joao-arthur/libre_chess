import type { MouseEvent, ReactElement } from "react";
import { useEffect, useRef } from "react";
import { useWindowDimension } from "../hooks/useWindowDimension";
import initWASM, { engineClick, EngineMatrixPoint } from "libre_chess_engine";
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

    function onClick(e: MouseEvent<HTMLCanvasElement>): void {
        // if (!model) {
        //     return;
        // }
        const row = e.pageY - e.currentTarget.offsetTop;
        const col = e.pageX - e.currentTarget.offsetLeft;
        try {
            engineClick(
                new EngineMatrixPoint(BigInt(Number(row)), BigInt(Number(col))),
            );
        } catch (e) {
            console.error(e);
        }
    }

    return (
        <main className="w-screen h-screen flex">
            <canvas
                onClick={onClick}
                className="m-auto"
                width={dimension}
                height={dimension}
                style={{ width: dimension, height: dimension }}
                ref={canvasRef}
            />
        </main>
    );
}
