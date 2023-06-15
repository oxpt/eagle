export interface TitleHeaderProps {
  title: string;
  subTitle: string;
}

export default function TitleHeader(props: TitleHeaderProps) {
  const { title, subTitle } = props;

  return (
    <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-4">
      <h1 className="text-3xl font-bold leading-tight tracking-tight text-gray-900">{title}</h1>
      <p className="text-gray-500">{subTitle}</p>
    </div>
  );
}
