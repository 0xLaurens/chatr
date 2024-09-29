import type {PageLoad} from './$types';
import {env} from "$env/dynamic/public";

export const load: PageLoad = async ({fetch}) => {
    try {
        let url = `${env.PUBLIC_API_URL}`;
        if (url.endsWith("/")) {
            url = url.slice(0, -1);
        }
        const res = await fetch(`${url}/rooms`);
        return await res.json();
    } catch (e) {
        return {
            status: "API offline (try again in a min)",
            rooms: []
        }
    }
}