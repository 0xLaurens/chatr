import type {PageLoad} from './$types';

export const load: PageLoad = async ({fetch}) => {
    const res = await fetch(`http://0.0.0.0:3000/rooms`);
    return await res.json();
}