import type { Type213 } from '../batch-01/file-0213.js';
export interface Type214 {
  id: 214;
  name: 'File214';
  next: Type213;
}

export function make214(): Type214 {
  return { id: 214, name: 'File214', next: null as unknown as Type213 };
}
