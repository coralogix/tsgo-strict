import type { Type3900 } from '../batch-19/file-3900.js';
export interface Type3901 {
  id: 3901;
  name: 'File3901';
  next: Type3900;
}

export function make3901(): Type3901 {
  return { id: 3901, name: 'File3901', next: null as unknown as Type3900 };
}
