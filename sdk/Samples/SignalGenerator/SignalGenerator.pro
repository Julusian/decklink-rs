lessThan(QT_MAJOR_VERSION, 5):error("SignalGenerator SDK sample requires Qt5")
equals(QT_MAJOR_VERSION, 5):lessThan(QT_MINOR_VERSION, 7):error("The SignalGenerator SDK sample requires at least Qt version 5.7")

QT       += core gui widgets

TARGET = SignalGenerator
TEMPLATE = app
CONFIG += c++11
INCLUDEPATH = ../../include
LIBS += -ldl

# The following define makes your compiler emit warnings if you use
# any feature of Qt which has been marked as deprecated (the exact warnings
# depend on your compiler). Please consult the documentation of the
# deprecated API in order to know how to port your code away from it.
DEFINES += QT_DEPRECATED_WARNINGS

# You can also make your code fail to compile if you use deprecated APIs.
# In order to do so, uncomment the following line.
# You can also select to disable deprecated APIs only up to a certain version of Qt.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

HEADERS 	=	SignalGenerator.h \
				SignalGeneratorEvents.h \
				com_ptr.h \
				DeckLinkDeviceDiscovery.h \
				DeckLinkOutputDevice.h \
				DeckLinkOpenGLWidget.h \
				ProfileCallback.h

SOURCES 	= 	main.cpp \
				../../include/DeckLinkAPIDispatch.cpp \
				DeckLinkDeviceDiscovery.cpp \
				DeckLinkOutputDevice.cpp \
				DeckLinkOpenGLWidget.cpp \
				SignalGenerator.cpp \
				ProfileCallback.cpp

FORMS 		= 	SignalGenerator.ui

