import type { Type998 } from '../batch-04/file-0998.js';
export interface Type999 {
  id: 999;
  name: 'File999';
  next: Type998;
}

export function make999(): Type999 {
  return { id: 999, name: 'File999', next: null as unknown as Type998 };
}
