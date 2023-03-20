import type {PageLoad} from './$types';
import {PUBLIC_API_URL} from "$env/static/public";

export const load: PageLoad = async ({fetch}) => {
    try {
        const res = await fetch(`http://${PUBLIC_API_URL}/rooms`);
        return await res.json();
    } catch (e) {
        return {
            status: "API offline (try again in a min)",
            rooms: []
        }
    }
}