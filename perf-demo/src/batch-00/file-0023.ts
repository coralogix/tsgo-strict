import type { Type22 } from '../batch-00/file-0022.js';
export interface Type23 {
  id: 23;
  name: 'File23';
  next: Type22;
}

export function make23(): Type23 {
  return { id: 23, name: 'File23', next: null as unknown as Type22 };
}
