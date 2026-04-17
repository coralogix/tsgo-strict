import type { Type3007 } from '../batch-15/file-3007.js';
export interface Type3008 {
  id: 3008;
  name: 'File3008';
  next: Type3007;
}

export function make3008(): Type3008 {
  return { id: 3008, name: 'File3008', next: null as unknown as Type3007 };
}
