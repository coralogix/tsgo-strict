import type { Type3030 } from '../batch-15/file-3030.js';
export interface Type3031 {
  id: 3031;
  name: 'File3031';
  next: Type3030;
}

export function make3031(): Type3031 {
  return { id: 3031, name: 'File3031', next: null as unknown as Type3030 };
}
