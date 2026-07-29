import ReactMarkdown from "react-markdown";
import type { Components } from "react-markdown";

const components: Components = {
  p: ({ children }) => <p className="mb-2 last:mb-0">{children}</p>,
  strong: ({ children }) => <strong className="font-semibold text-white">{children}</strong>,
  em: ({ children }) => <em className="italic">{children}</em>,
  ul: ({ children }) => <ul className="mb-2 ml-4 list-disc space-y-0.5 last:mb-0">{children}</ul>,
  ol: ({ children }) => <ol className="mb-2 ml-4 list-decimal space-y-0.5 last:mb-0">{children}</ol>,
  li: ({ children }) => <li>{children}</li>,
  // Headers get flattened to bold inline text so the AI's "##" habit doesn't blow up the bubble.
  h1: ({ children }) => <p className="mb-1 font-semibold text-white">{children}</p>,
  h2: ({ children }) => <p className="mb-1 font-semibold text-white">{children}</p>,
  h3: ({ children }) => <p className="mb-1 font-semibold text-white">{children}</p>,
  code: ({ children }) => (
    <code className="rounded bg-black/30 px-1 py-0.5 font-mono text-[0.85em]">{children}</code>
  ),
  a: ({ children, href }) => (
    <a href={href} target="_blank" rel="noreferrer" className="underline decoration-dotted">
      {children}
    </a>
  ),
};

export default function MessageContent({ content }: { content: string }) {
  return (
    <div className="[&_hr]:hidden">
      <ReactMarkdown components={components}>{content}</ReactMarkdown>
    </div>
  );
}
