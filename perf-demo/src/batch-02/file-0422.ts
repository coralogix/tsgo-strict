import type { Type421 } from '../batch-02/file-0421.js';
export interface Type422 {
  id: 422;
  name: 'File422';
  next: Type421;
}

export function make422(): Type422 {
  return { id: 422, name: 'File422', next: null as unknown as Type421 };
}
