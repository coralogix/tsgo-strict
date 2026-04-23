import type { Type501 } from '../batch-02/file-0501.js';
export interface Type502 {
  id: 502;
  name: 'File502';
  next: Type501;
}

export function make502(): Type502 {
  return { id: 502, name: 'File502', next: null as unknown as Type501 };
}
