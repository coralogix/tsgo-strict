import type { Type402 } from '../batch-02/file-0402.js';
export interface Type403 {
  id: 403;
  name: 'File403';
  next: Type402;
}

export function make403(): Type403 {
  return { id: 403, name: 'File403', next: null as unknown as Type402 };
}
