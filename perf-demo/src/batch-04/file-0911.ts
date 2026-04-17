import type { Type910 } from '../batch-04/file-0910.js';
export interface Type911 {
  id: 911;
  name: 'File911';
  next: Type910;
}

export function make911(): Type911 {
  return { id: 911, name: 'File911', next: null as unknown as Type910 };
}
