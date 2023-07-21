export interface PageTitleProps {
  title: string | null;
  description: string | null;
}

export default function PageTitle(props: PageTitleProps) {
  const { title, description } = props;
  return (
    <div className="bg-white py-4 sm:py-4">
      {title && (
        <h2 className="text-3xl font-bold leading-6 tracking-tight text-gray-900 sm:text-4xl sm:leading-5">
          {title}
        </h2>
      )}
      {description && (
        <p className="mt-4 text-base leading-7 text-gray-600">{description}</p>
      )}
    </div>
  );
}
