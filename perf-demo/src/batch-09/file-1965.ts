import type { Type1964 } from '../batch-09/file-1964.js';
export interface Type1965 {
  id: 1965;
  name: 'File1965';
  next: Type1964;
}

export function make1965(): Type1965 {
  return { id: 1965, name: 'File1965', next: null as unknown as Type1964 };
}
