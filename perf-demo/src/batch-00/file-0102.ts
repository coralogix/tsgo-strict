import type { Type101 } from '../batch-00/file-0101.js';
export interface Type102 {
  id: 102;
  name: 'File102';
  next: Type101;
}

export function make102(): Type102 {
  return { id: 102, name: 'File102', next: null as unknown as Type101 };
}
