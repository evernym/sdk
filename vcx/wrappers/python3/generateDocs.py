import os
import glob
import shutil


class FileTree(object):
    def __init__(self, dir, basepath, docpath):
        self.dir = dir 
        self.basepath = basepath
        print(self.basepath)
        print(basepath)
        self.docpath = docpath
        self.folders = self._get_folders(dir)
        self.files = glob.glob(dir + "*.py*")
        self._check_duplicate_files()



    def _check_duplicate_files(self):
        for file in self.files[:]:
            if file.endswith('.pyc'):
                if file.rstrip('c') in self.files:
                    self.files.remove(file)

    def _get_folders(self, path):
        out = []
        for file in glob.glob(path + "*"):
            if os.path.isdir(file):
                ifiletree = FileTree(file + "/", self.basepath, self.docpath)
                out.append(ifiletree)
        return out    

    def create_docs(self):
        for file in self.files:
            if file == 'pydoci.py':
                continue
            print("curwd : " + os.getcwd())
            print("pydoc3 -w " + os.path.join(self.basepath, file))
            os.system("pydoc3 -w " + os.path.join(self.basepath, file))

        for folder in self.folders:
            p = os.path.join(self.basepath, self.docpath, folder.dir)
            print('making directory: ' + p)
            os.makedirs(p)
            os.chdir(p)
            folder.create_docs()


if __name__ == '__main__':
    docs_path='docs'
    file_path='vcx'
    if os.path.isdir(os.path.join(os.getcwd(),docs_path)):
        print('removing old doc directory: %s' % os.path.join(os.getcwd(), docs_path))
        shutil.rmtree(os.path.join(os.getcwd(),docs_path))
    ft = FileTree("vcx/api", os.getcwd(), 'docs')
    ft.create_docs()
