import type { Type313 } from '../batch-01/file-0313.js';
export interface Type314 {
  id: 314;
  name: 'File314';
  next: Type313;
}

export function make314(): Type314 {
  return { id: 314, name: 'File314', next: null as unknown as Type313 };
}
