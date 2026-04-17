import type { Type420 } from '../batch-02/file-0420.js';
export interface Type421 {
  id: 421;
  name: 'File421';
  next: Type420;
}

export function make421(): Type421 {
  return { id: 421, name: 'File421', next: null as unknown as Type420 };
}
