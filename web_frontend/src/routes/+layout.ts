import { error } from "@sveltejs/kit";

export const prerender = true;
export const csr = true;

export async function load({ url }) {
    try {
        return {
            path: url.pathname,
        };
    } catch (err) {
        error(500, err);
    }
}
