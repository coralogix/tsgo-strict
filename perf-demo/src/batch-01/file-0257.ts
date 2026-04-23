import type { Type256 } from '../batch-01/file-0256.js';
export interface Type257 {
  id: 257;
  name: 'File257';
  next: Type256;
}

export function make257(): Type257 {
  return { id: 257, name: 'File257', next: null as unknown as Type256 };
}
