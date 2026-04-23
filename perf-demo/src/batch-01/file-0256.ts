import type { Type255 } from '../batch-01/file-0255.js';
export interface Type256 {
  id: 256;
  name: 'File256';
  next: Type255;
}

export function make256(): Type256 {
  return { id: 256, name: 'File256', next: null as unknown as Type255 };
}
