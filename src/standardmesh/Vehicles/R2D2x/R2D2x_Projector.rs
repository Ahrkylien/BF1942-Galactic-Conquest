subshader "R2D2x_Projector_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
        depthWrite false;
	twosided true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.7;
	texture "Mods\GCMOD\Movies\flagbeam";
}

subshader "R2D2x_Projector_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
        depthWrite false;
	twosided true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.5;
	texture "Mods\GCMOD\Movies\R2_projector2.bik";
}

