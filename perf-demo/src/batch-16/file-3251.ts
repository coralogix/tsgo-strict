import type { Type3250 } from '../batch-16/file-3250.js';
export interface Type3251 {
  id: 3251;
  name: 'File3251';
  next: Type3250;
}

export function make3251(): Type3251 {
  return { id: 3251, name: 'File3251', next: null as unknown as Type3250 };
}
