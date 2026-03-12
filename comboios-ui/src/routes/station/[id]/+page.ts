import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
  return {
    stationId: params.id
  };
};
