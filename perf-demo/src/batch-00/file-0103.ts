import type { Type102 } from '../batch-00/file-0102.js';
export interface Type103 {
  id: 103;
  name: 'File103';
  next: Type102;
}

export function make103(): Type103 {
  return { id: 103, name: 'File103', next: null as unknown as Type102 };
}
