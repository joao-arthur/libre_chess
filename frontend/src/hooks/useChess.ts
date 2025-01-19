import { useEffect, useState } from "react";
import {
    //engineAddOnChangeListener,
    //engineGetSettings,
    //EngineInfo,
    engineInit,
    engineSetDimension,
} from "libre_chess_engine";
import { useWindowDimension } from "./useWindowDimension";

type Chess = {
    readonly init: (canvasElement: HTMLCanvasElement) => void;
    //readonly model: EngineInfo | undefined;
};

export function useChess(): Chess {
    const [hasInited, setInit] = useState(false);
    //const [model, setModel] = useState<EngineInfo | undefined>(undefined);
    const dimension = useWindowDimension();

    useEffect(() => {
        if (hasInited && dimension > 0) {
            engineSetDimension(dimension);
        }
    }, [dimension, hasInited]);

    function init(canvasElement: HTMLCanvasElement) {
        const context = canvasElement.getContext("2d");
        if (!context) {
            return;
        }
        engineInit(context);
        //engineAddOnChangeListener(() => {
        //    let obj = engineGetSettings();
        //    setModel({
        //        size: obj.size,
        //    } as any);
        //});
        setInit(true);
    }

    return { 
        init,
        // model
         };
}
