import type { Type1339 } from '../batch-06/file-1339.js';
export interface Type1340 {
  id: 1340;
  name: 'File1340';
  next: Type1339;
}

export function make1340(): Type1340 {
  return { id: 1340, name: 'File1340', next: null as unknown as Type1339 };
}
