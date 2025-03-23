interface Window {
  documentPictureInPicture: DocumentPictureInPicture;
}

interface DocumentPictureInPicture {
  requestWindow(options: { width: number; height: number }): Promise<Window>;
}
