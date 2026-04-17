import type { Type3082 } from '../batch-15/file-3082.js';
export interface Type3083 {
  id: 3083;
  name: 'File3083';
  next: Type3082;
}

export function make3083(): Type3083 {
  return { id: 3083, name: 'File3083', next: null as unknown as Type3082 };
}
