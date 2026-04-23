import type { Type1338 } from '../batch-06/file-1338.js';
export interface Type1339 {
  id: 1339;
  name: 'File1339';
  next: Type1338;
}

export function make1339(): Type1339 {
  return { id: 1339, name: 'File1339', next: null as unknown as Type1338 };
}
