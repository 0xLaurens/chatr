import type {PageLoad} from './$types';

export const load: PageLoad = async ({fetch}) => {
    try {
        const res = await fetch(`http://localhost:3000/rooms`);
        return await res.json();
    } catch (e) {
        return {
            status: "API offline (try again in a min)",
            rooms: []
        }
    }
}