subshader "scopelight_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 0.5 1.0 0.5;
	materialSpecularPower 5.5;
	transparent true;
	twosided true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.7;
	texture "texture/Effects/scopelight";
}

