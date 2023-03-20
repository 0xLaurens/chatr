import type {PageLoad} from './$types';
import {env} from "$env/dynamic/public";

export const load: PageLoad = async ({fetch}) => {
    try {
        const res = await fetch(`http://${env.PUBLIC_API_URL}/rooms`);
        return await res.json();
    } catch (e) {
        return {
            status: "API offline (try again in a min)",
            rooms: []
        }
    }
}