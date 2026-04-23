import type { Type709 } from '../batch-03/file-0709.js';
export interface Type710 {
  id: 710;
  name: 'File710';
  next: Type709;
}

export function make710(): Type710 {
  return { id: 710, name: 'File710', next: null as unknown as Type709 };
}
