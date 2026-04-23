import type { Type99 } from '../batch-00/file-0099.js';
export interface Type100 {
  id: 100;
  name: 'File100';
  next: Type99;
}

export function make100(): Type100 {
  return { id: 100, name: 'File100', next: null as unknown as Type99 };
}
