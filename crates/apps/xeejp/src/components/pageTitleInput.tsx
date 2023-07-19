export interface PageTitleInputProps {
  title: string | null;
  description: string | null;
}

export default function PageTitleInput(props: PageTitleInputProps) {
  const { title, description } = props;
  return (
    <div className="grid px-1 py-4 sm:py-4">
      {title && (
        <input
          type="text"
          className="rounded-lg border-0 border-blue-500 bg-gray-50 text-3xl font-bold leading-6 tracking-tight text-gray-900 hover:border sm:text-4xl sm:leading-5"
          defaultValue={title}
        />
      )}
      {description && (
        <textarea
          className="mt-4 rounded-lg border-0 border-blue-500 bg-gray-50 text-base leading-7 text-gray-600 hover:border"
          defaultValue={description}
          rows={1}
        />
      )}
    </div>
  );
}
