TEMPLATE  	= app
LANGUAGE  	= C++
CONFIG		+= qt opengl
QT			+= opengl
INCLUDEPATH =	../../include ../NVIDIA_GPUDirect/include
contains(QT_ARCH, x86_64) {
	LIBDVP_PATH		= ../NVIDIA_GPUDirect/x86_64
} else {
	LIBDVP_PATH		= ../NVIDIA_GPUDirect/i386
}
LIBS				+= -lGLU -ldl -ldvp -L$$LIBDVP_PATH -Wl,-rpath=.:$$LIBDVP_PATH
QMAKE_PRE_LINK		+= $$QMAKE_SYMBOLIC_LINK libdvp.so.1 $$LIBDVP_PATH/libdvp.so

HEADERS 	=	../../include/DeckLinkAPIDispatch.cpp \
				LoopThroughWithOpenGLCompositing.h \
				OpenGLComposite.h \
				GLExtensions.h \
				VideoFrameTransfer.h

SOURCES 	= 	main.cpp \
				../../include/DeckLinkAPIDispatch.cpp \
				LoopThroughWithOpenGLCompositing.cpp \
				OpenGLComposite.cpp \
				GLExtensions.cpp \
				VideoFrameTransfer.cpp

FORMS 		= 	LoopThroughWithOpenGLCompositing.ui
