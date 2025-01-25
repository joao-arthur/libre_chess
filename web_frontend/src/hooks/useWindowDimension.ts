import { useEffect, useState } from "react";

export function useWindowDimension(): number {
    const [dimension, setDimension] = useState(0);

    useEffect(() => {
        function onWindowResize(): void {
            setDimension(Math.min(window.innerWidth, window.innerHeight));
        }
        onWindowResize();
        window.addEventListener("resize", onWindowResize);
        return () => {
            window.removeEventListener("resize", onWindowResize);
        };
    }, []);

    return dimension;
}
