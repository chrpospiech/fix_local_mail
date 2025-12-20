######################################
# Find KDE include dirs and libs     #
######################################

find_package(ECM 0.0.11 REQUIRED NO_MODULE)

list(APPEND CMAKE_MODULE_PATH
	${ECM_MODULE_PATH}
	${ECM_KDE_MODULE_DIR} )

include(KDEInstallDirs)
include(KDECMakeSettings)
include(KDECompilerSettings)
include(FeatureSummary)

find_package(Qt5 REQUIRED COMPONENTS Core)
find_package(KF5Akonadi REQUIRED)
find_package(KPim5Mime REQUIRED)

######################################
# Adding Qt5 and KDE include dirs    #
######################################

if (Qt5_FOUND)
  message(STATUS "Adding include files and libraries for Qt5.")
  include_directories(${Qt5Core_INCLUDE_DIRS},
                      ${Qt5Gui_INCLUDE_DIRS},
                      ${QtWidgets_INCLUDE_DIRS})

  if (Qt5_POSITION_INDEPENDENT_CODE)
    message(STATUS
      "Adding ${Qt5Core_EXECUTABLE_COMPILE_FLAGS} to CXXFLAGS.")
    SET( CMAKE_CXX_FLAGS
      "${CMAKE_CXX_FLAGS} ${Qt5Core_EXECUTABLE_COMPILE_FLAGS}" )
    SET( CMAKE_EXE_LINKER_FLAGS
      "${CMAKE_EXE_LINKER_FLAGS} ${Qt5Core_EXECUTABLE_COMPILE_FLAGS}" )
  endif (Qt5_POSITION_INDEPENDENT_CODE)
else (Qt5_FOUND)
  message(FATAL_ERROR "Qt5 was not found.")
endif (Qt5_FOUND)

if (KF5Akonadi_FOUND AND KPim5Mime_FOUND)
  message(STATUS "Adding include files and libraries for KF5.")
  include_directories(
    ${KF5_INCLUDE_INSTALL_DIR}/AkonadiCore,
    ${KF5_INCLUDE_INSTALL_DIR}/AkonadiMime
  )
else (KF5Akonadi_FOUND AND KPim5Mime_FOUND)
  message(FATAL_ERROR "KF5Akonadi or KPim5Mime was not found.")
endif (KF5Akonadi_FOUND AND KPim5Mime_FOUND)

######################################
# Names of executable and library    #
######################################

set(CMAKE_CXX_STANDARD 17)

# Build as shared library
add_library(akonadi_helper SHARED
    src/akonadi_lib.cpp
)

# Set visibility to export symbols
set_target_properties(akonadi_helper PROPERTIES
    CXX_VISIBILITY_PRESET default
    VISIBILITY_INLINES_HIDDEN OFF
)

target_include_directories(akonadi_helper PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

target_link_libraries(akonadi_helper
    Qt5::Core
    KF5::AkonadiCore
    KPim5::Mime
)

install(TARGETS akonadi_helper DESTINATION ${CMAKE_CURRENT_SOURCE_DIR}/../lib)
