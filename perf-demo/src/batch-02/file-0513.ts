import type { Type512 } from '../batch-02/file-0512.js';
export interface Type513 {
  id: 513;
  name: 'File513';
  next: Type512;
}

export function make513(): Type513 {
  return { id: 513, name: 'File513', next: null as unknown as Type512 };
}
