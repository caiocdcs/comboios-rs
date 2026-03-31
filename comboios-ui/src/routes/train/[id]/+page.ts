import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { getTrainJourney } from '$lib/api';
import type { TrainDetails } from '$lib/types';

export const load: PageLoad = async ({ params, url }) => {
  const trainNumber = params.id;
  const dateParam = url.searchParams.get('date');
  
  let date: string;
  if (dateParam && dateParam !== 'undefined' && dateParam !== 'null') {
    const parts = dateParam.split('-');
    if (parts.length === 3 && parts[0].length === 2) {
      date = `${parts[2]}-${parts[1]}-${parts[0]}`;
    } else if (/^\d{4}-\d{2}-\d{2}$/.test(dateParam)) {
      date = dateParam;
    } else {
      date = new Date().toISOString().split('T')[0];
    }
  } else {
    date = new Date().toISOString().split('T')[0];
  }
  
  try {
    const train = await getTrainJourney(trainNumber, date);
    return { train };
  } catch (e) {
    const message = e instanceof Error ? e.message : 'Unknown error';
    throw error(500, message);
  }
};
