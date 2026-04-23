import type { Type21 } from '../batch-00/file-0021.js';
export interface Type22 {
  id: 22;
  name: 'File22';
  next: Type21;
}

export function make22(): Type22 {
  return { id: 22, name: 'File22', next: null as unknown as Type21 };
}
